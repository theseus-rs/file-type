use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48623384: FileFormat = FileFormat {
    id: 48_623_384,
    puid: "wikidata/48623384",
    name: "Microsoft Project Export File",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[],
    related_formats: &[],
};
