use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51334664: FileFormat = FileFormat {
    id: 51_334_664,
    puid: "wikidata/51334664",
    name: "Microsoft Powerpoint Presentation, version 4",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
