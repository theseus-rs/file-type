use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205619: FileFormat = FileFormat {
    id: 28_205_619,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 2 Hot Icon",
    extensions: &["bmh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
