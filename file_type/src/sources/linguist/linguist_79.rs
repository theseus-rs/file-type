use crate::format::FileFormat;

pub(crate) const LINGUIST_79: FileFormat = FileFormat {
    id: 79,
    puid: "linguist/79",
    name: "Cython",
    extensions: &["pxd", "pxi", "pyx"],
    media_types: &["text/x-cython"],
    internal_signatures: &[],
    related_formats: &[],
};
