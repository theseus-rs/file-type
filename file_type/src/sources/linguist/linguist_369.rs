use crate::format::FileFormat;

pub(crate) const LINGUIST_369: FileFormat = FileFormat {
    id: 369,
    puid: "linguist/369",
    name: "TeX",
    extensions: &[
        "aux", "bbx", "cbx", "cls", "dtx", "ins", "lbx", "ltx", "mkii", "mkiv", "mkvi", "sty",
        "tex", "toc",
    ],
    media_types: &["text/x-stex"],
    internal_signatures: &[],
    related_formats: &[],
};
