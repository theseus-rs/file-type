use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49243748: FileFormat = FileFormat {
    id: 49_243_748,
    puid: "wikidata/49243748",
    name: "Acrobat Language definition file",
    extensions: &["lng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
