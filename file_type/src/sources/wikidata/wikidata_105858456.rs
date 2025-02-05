use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858456: FileFormat = FileFormat {
    id: 105_858_456,
    source_type: SourceType::Wikidata,
    name: "Acrobat eBook Reader EBX Transfer Data",
    extensions: &["etd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
