use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27902177: FileFormat = FileFormat {
    id: 27_902_177,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.2",
    extensions: &["adx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
