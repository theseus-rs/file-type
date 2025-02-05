use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43974596: FileFormat = FileFormat {
    id: 43_974_596,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (Binary), version R13",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
