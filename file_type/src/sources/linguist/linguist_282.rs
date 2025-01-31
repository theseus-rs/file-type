use crate::format::FileFormat;

pub(crate) const LINGUIST_282: FileFormat = FileFormat {
    id: 282,
    puid: "linguist/282",
    name: "Perl",
    extensions: &[
        "al", "cgi", "fcgi", "perl", "ph", "pl", "plx", "pm", "psgi", "t",
    ],
    media_types: &["text/x-perl"],
    internal_signatures: &[],
    related_formats: &[],
};
