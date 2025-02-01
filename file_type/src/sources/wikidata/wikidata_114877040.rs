use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114877040: FileFormat = FileFormat {
    id: 114_877_040,
    puid: "wikidata/114877040",
    name: "Microsoft Money Backup File",
    extensions: &["mbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
