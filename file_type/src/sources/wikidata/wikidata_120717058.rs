use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120717058: FileFormat = FileFormat {
    id: 120_717_058,
    source_type: SourceType::Wikidata,
    name: "DeductionPro 2007 Data File",
    extensions: &["d07"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
