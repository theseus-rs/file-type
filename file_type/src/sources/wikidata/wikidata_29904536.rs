use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904536: FileFormat = FileFormat {
    id: 29_904_536,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System stored program",
    extensions: &["sas7bpgm", "ss2", "ss7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
