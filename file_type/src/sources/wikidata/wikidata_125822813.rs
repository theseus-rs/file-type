use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125822813: FileFormat = FileFormat {
    id: 125_822_813,
    puid: "wikidata/125822813",
    name: "Microsoft Help Combined Full-text Search file",
    extensions: &["chq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
