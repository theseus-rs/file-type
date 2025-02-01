use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47165323: FileFormat = FileFormat {
    id: 47_165_323,
    puid: "wikidata/47165323",
    name: "Microsoft Project file format, version 2000-2003",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
