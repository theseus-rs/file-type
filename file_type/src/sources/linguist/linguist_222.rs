use crate::format::FileFormat;

pub(crate) const LINGUIST_222: FileFormat = FileFormat {
    id: 222,
    puid: "linguist/222",
    name: "Markdown",
    extensions: &[
        "livemd", "markdown", "md", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn", "scd",
        "workbook",
    ],
    media_types: &["text/x-gfm"],
    internal_signatures: &[],
    related_formats: &[],
};
