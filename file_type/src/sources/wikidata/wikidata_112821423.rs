use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112821423: FileFormat = FileFormat {
    id: 112_821_423,
    source_type: SourceType::Wikidata,
    name: "Minolta 3D Scanner Element File",
    extensions: &["vvd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
