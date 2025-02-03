use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131062662: FileFormat = FileFormat {
    id: 131_062_662,
    source_type: SourceType::Wikidata,
    name: "SNOBOL4 file format",
    extensions: &["snobol"],
    media_types: &["text/x-snobol"],
    internal_signatures: &[],
    related_formats: &[],
};
