use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333907: FileFormat = FileFormat {
    id: 111_333_907,
    source_type: SourceType::Wikidata,
    name: "Roland MT-32 Control + PCM ROM dumps",
    extensions: &["rom"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
