use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904536: FileFormat = FileFormat {
    id: 29_904_536,
    puid: "wikidata/29904536",
    name: "Statistical Analysis System stored program",
    extensions: &["sas7bpgm", "ss2", "ss7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
