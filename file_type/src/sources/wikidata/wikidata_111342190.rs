use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342190: FileFormat = FileFormat {
    id: 111_342_190,
    puid: "wikidata/111342190",
    name: "Avalon sample",
    extensions: &["smp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
