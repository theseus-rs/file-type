use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34308624: FileFormat = FileFormat {
    id: 34_308_624,
    source_type: SourceType::Wikidata,
    name: "Scribe manuscript",
    extensions: &["mss"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
