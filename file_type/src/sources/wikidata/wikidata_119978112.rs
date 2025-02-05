use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119978112: FileFormat = FileFormat {
    id: 119_978_112,
    source_type: SourceType::Wikidata,
    name: "Clip File",
    extensions: &["fmc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
