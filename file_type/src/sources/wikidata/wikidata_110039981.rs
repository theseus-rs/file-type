use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110039981: FileFormat = FileFormat {
    id: 110_039_981,
    puid: "wikidata/110039981",
    name: "Phantom CINE Compressed Video File",
    extensions: &["cci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
