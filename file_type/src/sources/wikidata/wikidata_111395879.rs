use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111395879: FileFormat = FileFormat {
    id: 111_395_879,
    source_type: SourceType::Wikidata,
    name: "FloppyShot Image",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
