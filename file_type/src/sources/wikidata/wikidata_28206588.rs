use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206588: FileFormat = FileFormat {
    id: 28_206_588,
    puid: "wikidata/28206588",
    name: "Microsoft Image Composer file",
    extensions: &["mic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
