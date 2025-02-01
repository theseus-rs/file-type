use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131453299: FileFormat = FileFormat {
    id: 131_453_299,
    puid: "wikidata/131453299",
    name: "YARA file format",
    extensions: &["yar"],
    media_types: &["text/x-yara"],
    internal_signatures: &[],
    related_formats: &[],
};
