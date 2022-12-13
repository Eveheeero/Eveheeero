tokio::task_local! {
    static DATA1: u128;
    static DATA2: &'static str;
    static DATA3: String;
}

pub(super) fn localkey() {
    DATA1.sync_scope(1, || {
        println!("localkey: {}", DATA1.get());
    });

    DATA2.sync_scope("data", || {
        println!("localkey: {}", DATA2.get());
    });

    // String은 copy불가능하기때문에 복사가안됨, &'static형태도 마찬가지임
    // DATA3.sync_scope("data".to_string(), || {
    //     println!("localkey: {}", DATA3.get());
    // });

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        DATA1
            .scope(4, async move {
                println!("localkey: {}", DATA1.get());
            })
            .await;
    });
}
