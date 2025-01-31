use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125823475: FileFormat = FileFormat {
    id: 125_823_475,
    puid: "wikidata/125823475",
    name: "Microsoft Help merged query index file",
    extensions: &["hxq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
