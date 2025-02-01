use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128694654: FileFormat = FileFormat {
    id: 128_694_654,
    puid: "wikidata/128694654",
    name: "Carbon file format",
    extensions: &["carbon"],
    media_types: &["text/x-carbon"],
    internal_signatures: &[],
    related_formats: &[],
};
