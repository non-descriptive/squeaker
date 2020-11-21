
pub enum LineType {
    RenderLine, // i - basic non-functional
    File      , // 0   Item is a file
    Dir       , // 1   Item is a directory
    Phonebook , // 2   Item is a CSO phone-book server
    Error     , // 3   Error
    BinHex    , // 4   Item is a BinHexed Macintosh file.
    DosBin    , // 5   Item is DOS binary archive of some sort. Client must read until the TCP connection closes.  Beware.
    Uuencoded , // 6   Item is a UNIX uuencoded file.
    Index     , // 7   Item is an Index-Search server.
    Telnet    , // 8   Item points to a text-based telnet session.
    Binary    , // 9   Item is a binary file! Client must read until the TCP connection closes.  Beware.
    Redundant , // +   Item is a redundant server
    Tn3270    , // T   Item points to a text-based tn3270 session.
    Gif       , // g   Item is a GIF format graphics file.
    Image     , // I   Item is some kind of image file.  Client decides how to display.
    Undefined , // other unhandled type
}
impl From<&str> for LineType {
    fn from(slice: &str) -> Self {
        match slice {
            "i" => LineType::RenderLine,
            "0" => LineType::File,
            "1" => LineType::Dir,
            "2" => LineType::Phonebook,
            "3" => LineType::Error,
            "4" => LineType::BinHex,
            "5" => LineType::DosBin,
            "6" => LineType::Uuencoded,
            "7" => LineType::Index,
            "8" => LineType::Telnet,
            "9" => LineType::Binary,
            "+" => LineType::Redundant,
            "T" => LineType::Tn3270,
            "g" => LineType::Gif,
            "I" => LineType::Image,
            _ => LineType::Undefined
        }
    }
}
