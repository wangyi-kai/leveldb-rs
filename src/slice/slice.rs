
/// Slice 对象
#[derive(Copy, Clone)]
pub struct Slice<'a> {
    /// Slice 数据
    pub data: &'a [u8],
    /// Slice 大小
    pub len: usize,
}

impl<'a> Slice<'a> {
    /// 创建一个空的 Slice 对象
    pub fn new() -> Self {
        Self {
            data: &[],
            len: 0,
        }
    }

    /// 从字节数组创建一个 Slice 对象
    pub fn from_bytes(data: &'a [u8]) -> Self {
        Self {
            data,
            len: data.len(),
        }
    }

    /// 从字符串创建一个 Slice 对象
    pub fn from_string(data: &'a str) -> Self {
        Self {
            data: data.as_bytes(),
            len: data.len(),
        }
    }

    /// 获取 Slice 对象的数据
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// 获取 Slice 对象的大小
    pub fn size(&self) -> usize {
        self.len
    }

    /// 判断 Slice 是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 获取 Slice 的第 index 个字节
    pub fn get_bytes(&self, index: usize) -> u8 {
        self.data[index]
    }

    /// 清空 Slice
    pub fn clear(&mut self) {
        self.data = &[];
        self.len = 0;
    }


    /// 移除 Slice 的前 n 个字节
    pub fn remove_prefix(&mut self, n: usize) {
        assert!(n <= self.len);
        self.data = &self.data[n..];
        self.len -= n;
    }

    /// 获取 Slice 的字符串表示
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.data).to_string()
    }

    /// 比较两个 Slice
    pub fn compare(&self, other: &Slice) -> i32 {
        match self.data.cmp(other.data) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// 判断 Slice 是否以 prefix 开头
    pub fn starts_with(&self, prefix: &Slice) -> bool {
        self.data.starts_with(prefix.data)
    }
}