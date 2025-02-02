use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858456: FileFormat = FileFormat {
    id: 105_858_456,
    source_type: SourceType::Wikidata,
    name: "Acrobat eBook Reader EBX Transfer Data",
    extensions: &["etd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
