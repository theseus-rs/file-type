use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5531823: FileFormat = FileFormat {
    id: 5_531_823,
    puid: "wikidata/5531823",
    name: "General Data Format for Biomedical Signals",
    extensions: &["gdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
