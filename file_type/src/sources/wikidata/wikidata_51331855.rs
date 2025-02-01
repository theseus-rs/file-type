use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51331855: FileFormat = FileFormat {
    id: 51_331_855,
    puid: "wikidata/51331855",
    name: "Microsoft Powerpoint Design Template",
    extensions: &["pot"],
    media_types: &["application/mspowerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
