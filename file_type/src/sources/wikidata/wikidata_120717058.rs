use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120717058: FileFormat = FileFormat {
    id: 120_717_058,
    source_type: SourceType::Wikidata,
    name: "DeductionPro 2007 Data File",
    extensions: &["d07"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
