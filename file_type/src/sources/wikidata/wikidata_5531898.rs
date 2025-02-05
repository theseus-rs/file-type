use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5531898: FileFormat = FileFormat {
    id: 5_531_898,
    source_type: SourceType::Wikidata,
    name: "General Exchange Format",
    extensions: &["gxf"],
    media_types: &["application/gxf"],
    signatures: &[],
    related_formats: &[],
};
