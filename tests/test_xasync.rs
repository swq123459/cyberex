#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use cyberex::async_call;
    use tokio::sync::{mpsc::unbounded_channel, oneshot};

    #[tokio::test]
    async fn test_case_send_call_enum() {
        enum Ctrl {
            Start {
                name: String,
                age: i32,
                ret: oneshot::Sender<String>,
            },
            Pause(String, oneshot::Sender<()>),
            Continue {
                name: String,
                ret: oneshot::Sender<String>,
            },
            Reset(String, i32, oneshot::Sender<String>),
            End {
                ret: oneshot::Sender<()>,
            },
        }

        let (ctrl_tx, mut ctrl_rx) = unbounded_channel();
        let th = tokio::spawn(async move {
            while let Some(cmd) = ctrl_rx.recv().await {
                match cmd {
                    Ctrl::Start { name, age, ret } => {
                        let _ = ret.send("world".to_string());
                    },
                    Ctrl::Reset(name, age, ret) => {
                        let _ = ret.send("world".to_string());
                    },
                    Ctrl::Pause(_, ret) => {
                        let _ = ret.send(());
                    },
                    Ctrl::Continue { name, ret } => {
                        let _ = ret.send("world".to_string());
                    },

                    Ctrl::End { ret } => {
                        let _ = ret.send(());
                        return;
                    },
                }
            }
        });
        assert_eq!(
            async_call!(
                ctrl_tx,
                Ctrl::Start {
                    name: "hello".to_string(),
                    age: 12
                }
            )
            .await,
            Ok("world".to_string())
        );
        assert_eq!(
            {
                let name = "hello".to_string();
                async_call!(ctrl_tx, Ctrl::Start { name, age: 12 }).await
            },
            Ok("world".to_string())
        );
        assert_eq!(
            {
                let name = "hello".to_string();
                let age = 12;
                async_call!(ctrl_tx, Ctrl::Start { name, age }).await
            },
            Ok("world".to_string())
        );
        assert_eq!(
            {
                let name = "hello".to_string();
                async_call!(ctrl_tx, Ctrl::Continue { name }).await
            },
            Ok("world".to_string())
        );
        assert_eq!(
            async_call!(ctrl_tx, Ctrl::Reset("hello".to_string(), 12)).await,
            Ok("world".to_string())
        );
        assert_eq!(async_call!(ctrl_tx, Ctrl::Pause("hello".to_string())).await, Ok(()));

        assert_eq!(async_call!(ctrl_tx, Ctrl::End {}).await, Ok(()));
        let _ = th.await;
    }
    #[tokio::test]
    async fn test_case_send_call_enum_in_namespace() {
        mod test_detail {
            use tokio::sync::oneshot;

            pub enum Ctrl {
                End { ret: oneshot::Sender<()> },
                Pause(String, oneshot::Sender<()>),
            }
        }

        let (ctrl_tx, mut ctrl_rx) = unbounded_channel();
        let th = tokio::spawn(async move {
            while let Some(cmd) = ctrl_rx.recv().await {
                match cmd {
                    test_detail::Ctrl::End { ret } => {
                        let _ = ret.send(());
                        return;
                    },
                    test_detail::Ctrl::Pause(_, ret) => {
                        let _ = ret.send(());
                    },
                }
            }
        });
        assert_eq!(
            async_call!(ctrl_tx, test_detail::Ctrl::Pause("hello".to_string())).await,
            Ok(())
        );

        assert_eq!(async_call!(ctrl_tx, test_detail::Ctrl::End {}).await, Ok(()));
        let _ = th.await;
    }

    #[tokio::test]
    async fn test_case_send_call_just_struct() {
        struct JustStrut {
            name: String,
            ret: oneshot::Sender<()>,
        }
        let (ctrl_tx, mut ctrl_rx) = unbounded_channel();
        let th = tokio::spawn(async move {
            if let Some(JustStrut { name, ret }) = ctrl_rx.recv().await {
                let _ = ret.send(());
            }
        });
        assert_eq!(
            async_call!(
                ctrl_tx,
                JustStrut {
                    name: "hello".to_string()
                }
            )
            .await,
            Ok(())
        )
    }
    #[tokio::test]
    async fn test_case_send_call_just_struct_muti_field() {
        struct JustStrut {
            name: String,
            age: i32,
            ret: oneshot::Sender<()>,
        }
        let (ctrl_tx, mut ctrl_rx) = unbounded_channel();
        let th = tokio::spawn(async move {
            while let Some(JustStrut { name, age, ret }) = ctrl_rx.recv().await {
                let _ = ret.send(());
            }
        });
        assert_eq!(
            async_call!(
                ctrl_tx,
                JustStrut {
                    name: "hello".to_string(),
                    age: 12
                }
            )
            .await,
            Ok(())
        );

        {
            let name = "hello".to_string();
            assert_eq!(async_call!(ctrl_tx, JustStrut { name, age: 12 }).await, Ok(()));
        }
        {
            let name = "hello".to_string();
            let age = 12;
            assert_eq!(async_call!(ctrl_tx, JustStrut { name, age }).await, Ok(()));
        }

        th.abort();
    }
    #[tokio::test]
    async fn test_case_send_call_just_struct_in_namespace() {
        mod test_detail {
            pub struct JustStrut {
                pub name: String,
                pub ret: tokio::sync::oneshot::Sender<()>,
            }
        }

        let (ctrl_tx, mut ctrl_rx) = unbounded_channel();
        let th = tokio::spawn(async move {
            if let Some(test_detail::JustStrut { name, ret }) = ctrl_rx.recv().await {
                let _ = ret.send(());
            }
        });
        assert_eq!(
            async_call!(
                ctrl_tx,
                test_detail::JustStrut {
                    name: "hello".to_string()
                }
            )
            .await,
            Ok(())
        )
    }
}
