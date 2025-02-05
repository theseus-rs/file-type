use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72727591: FileFormat = FileFormat {
    id: 72_727_591,
    source_type: SourceType::Wikidata,
    name: "Juno address book",
    extensions: &["nv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
