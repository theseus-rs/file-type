use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127784636: FileFormat = FileFormat {
    id: 127_784_636,
    source_type: SourceType::Wikidata,
    name: "Metal Shading Language File",
    extensions: &["metal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
