use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34308624: FileFormat = FileFormat {
    id: 34_308_624,
    source_type: SourceType::Wikidata,
    name: "Scribe manuscript",
    extensions: &["mss"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
