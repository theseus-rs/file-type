use crate::format::FileFormat;

pub(crate) const LINGUIST_245: FileFormat = FileFormat {
    id: 245,
    puid: "linguist/245",
    name: "NetLinx+ERB",
    extensions: &["axi.erb", "axs.erb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
