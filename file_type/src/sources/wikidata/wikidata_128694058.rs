use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128694058: FileFormat = FileFormat {
    id: 128_694_058,
    puid: "wikidata/128694058",
    name: "OpenBugs file",
    extensions: &["bug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
