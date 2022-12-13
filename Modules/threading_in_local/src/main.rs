mod tokio_localkey;
mod tokio_localset;

fn main() {
    tokio_localset::localset();
    tokio_localkey::localkey();
    /*
    std::scope
    crossbeam::scope
    futures::executor::block_on
    scopeguard::defer
    등등의 내용은 작성되지 않음, 추후 필요시 작성
    */
}
