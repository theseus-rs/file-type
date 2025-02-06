use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131453299: FileFormat = FileFormat {
    id: 131_453_299,
    source_type: SourceType::Wikidata,
    name: "YARA file format",
    extensions: &["yar"],
    media_types: &["text/x-yara"],
    signatures: &[],
    related_formats: &[],
};
