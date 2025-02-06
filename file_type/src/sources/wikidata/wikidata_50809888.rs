use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50809888: FileFormat = FileFormat {
    id: 50_809_888,
    source_type: SourceType::Wikidata,
    name: "Google Document Link File",
    extensions: &[
        "gdoc", "gdraw", "gform", "gmap", "gsheet", "gsite", "gslides",
    ],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
