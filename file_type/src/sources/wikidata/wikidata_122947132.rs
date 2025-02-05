use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122947132: FileFormat = FileFormat {
    id: 122_947_132,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange File Format (ASCII), version 2010-2012",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[],
    related_formats: &[],
};
