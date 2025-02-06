use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867518: FileFormat = FileFormat {
    id: 105_867_518,
    source_type: SourceType::Wikidata,
    name: "Haines NFF scene",
    extensions: &["nff"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
