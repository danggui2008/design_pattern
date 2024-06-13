//图像接口【抽象角色】
trait Image {
    fn display(&self);
}

//图片【真实角色】
struct RealImage {
    file_name: String,
}
//RealImageImage接口
impl RealImage {
    fn new<T>(file_name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            file_name: file_name.into(),
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("displaying  image {}", self.file_name);
    }
}

//代理图片【代理角色】
struct ProxyImage {
    file_name: String,
    real_image: RealImage,
}

impl ProxyImage {
    fn new<T>(file_name: T,real_image:RealImage) -> Self
    where
        T: Into<String>,
    {
        Self {
            file_name: file_name.into(),
            real_image,
        }
    }
}
//代理对象实现Image接口
impl Image for ProxyImage {
    fn display(&self) {
        
        self.real_image.display();
    }
}

#[test]
fn main() {
    let proxy_image = ProxyImage::new("image1.png",RealImage::new("image1.png"));
    // 图像未加载，直到调用display()方法
    proxy_image.display();
    //图片已加载
    proxy_image.display();
}
