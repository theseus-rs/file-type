use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47165003: FileFormat = FileFormat {
    id: 47_165_003,
    puid: "wikidata/47165003",
    name: "Microsoft Project file format, version 4",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
