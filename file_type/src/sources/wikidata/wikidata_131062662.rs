use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131062662: FileFormat = FileFormat {
    id: 131_062_662,
    puid: "wikidata/131062662",
    name: "SNOBOL4 file format",
    extensions: &["snobol"],
    media_types: &["text/x-snobol"],
    internal_signatures: &[],
    related_formats: &[],
};
