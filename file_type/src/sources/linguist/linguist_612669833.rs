use crate::format::FileFormat;

pub(crate) const LINGUIST_612669833: FileFormat = FileFormat {
    id: 612_669_833,
    puid: "linguist/612669833",
    name: "Roff Manpage",
    extensions: &[
        "1", "1in", "1m", "1x", "2", "3", "3in", "3m", "3p", "3pm", "3qt", "3x", "4", "5", "6",
        "7", "8", "9", "man", "mdoc",
    ],
    media_types: &["text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
