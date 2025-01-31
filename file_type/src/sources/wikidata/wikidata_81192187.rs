use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81192187: FileFormat = FileFormat {
    id: 81_192_187,
    puid: "wikidata/81192187",
    name: "Microsoft Border art",
    extensions: &["bdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
