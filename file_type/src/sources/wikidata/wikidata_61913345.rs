use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61913345: FileFormat = FileFormat {
    id: 61_913_345,
    puid: "wikidata/61913345",
    name: "Microsoft Project Export File, version 3",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[],
    related_formats: &[],
};
