use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47165026: FileFormat = FileFormat {
    id: 47_165_026,
    puid: "wikidata/47165026",
    name: "Microsoft Project file format, version 95",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
