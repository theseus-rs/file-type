use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131062662: FileFormat = FileFormat {
    id: 131_062_662,
    source_type: SourceType::Wikidata,
    name: "SNOBOL4 file format",
    extensions: &["snobol"],
    media_types: &["text/x-snobol"],
    signatures: &[],
    related_formats: &[],
};
