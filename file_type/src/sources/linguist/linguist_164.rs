use crate::format::FileFormat;

pub(crate) const LINGUIST_164: FileFormat = FileFormat {
    id: 164,
    puid: "linguist/164",
    name: "IRC log",
    extensions: &["irclog", "weechatlog"],
    media_types: &["text/mirc"],
    internal_signatures: &[],
    related_formats: &[],
};
