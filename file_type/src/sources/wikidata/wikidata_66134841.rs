use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66134841: FileFormat = FileFormat {
    id: 66_134_841,
    source_type: SourceType::Wikidata,
    name: "ACCDA file format",
    extensions: &["accda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
